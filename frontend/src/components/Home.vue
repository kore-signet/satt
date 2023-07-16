<script setup lang="ts">
import * as api from '../api'
import { computed, reactive, ref, watch, type Ref } from 'vue'
import Episode from './Episode.vue'
import Form from './Form.vue'
import { QueryKind, type ApiRequest, type ApiResponse, type EpisodeData, type ApiResult } from '../types'
import { useRouter, useRoute } from 'vue-router'
import { onMounted } from 'vue'
import { array_unordered_equals, duration_humanizer } from '@/utils'


type ResultData = { episodes: EpisodeData[], query_took: number, page: string | null }

const made_request = ref(false);

const results = reactive<ResultData>({
  episodes: [],
  query_took: 0,
  page: null
})

const result_err = ref<string | null>(null);

const router = useRouter()
const route = useRoute()

const request = reactive<ApiRequest>({
  query: route.query['q'] as string || '',
  kind: route.query['kind'] as QueryKind || QueryKind.PHRASE,
  seasons: route.query['seasons'] ? (route.query['seasons'] as string).split(',') : [],
  highlight: true
})

watch(route, (route) => {
  request.query = route.query['q'] as string || request.query || ''
  request.kind = route.query['kind'] as QueryKind || request.kind || QueryKind.PHRASE
  request.seasons = (route.query['seasons'] ? (route.query['seasons'] as string).split(',') : []) || request.seasons || []
})

onMounted(async () => {
  if (request.query != '') {
    await search()
  }
})

function track_search() {
  if (process.env.NODE_ENV == "development") {
    return;
  }

  try {
    (<any>window).umami.track('search', {
      query: request.query,
      seasons: request.seasons || [],
      query_kind: request.kind
    })
  } catch (err) {
    console.log(err)
  }
}

async function search() {
  made_request.value = true;

  let new_query: Record<string, any> = {
    'q': request.query,
    'kind': request.kind,
  };

  if (request.seasons && request.seasons.length > 0) {
    if (!array_unordered_equals(request.seasons, api.all_seasons)) {
      new_query['seasons'] = request.seasons.join(',');
    }
  }

  router.push({
    path: '/', query: new_query
  });


  let res = await api.search(request)

  if (res.ok) {
    result_err.value = null;

    results.episodes = res.value.episodes
    results.query_took = res.value.query_time
    results.page = res.value.next_page
  } else {
    result_err.value = res.msg;
  }

}

async function load_more() {
  made_request.value = true;

  let res: ApiResult = await api.search({ page: results.page })

  if (res.ok) {
    result_err.value = null;

    results.page = res.value.next_page
    results.episodes = results.episodes.concat(res.value.episodes)
    results.query_took = res.value.query_time
  } else {
    result_err.value = res.msg;
  }

}

const total_highlights = computed(() => {
  return results.episodes.reduce((acc, ep) => acc + ep.highlights.length, 0)
})

const time_taken = computed(() => {
  return duration_humanizer(results.query_took * 1000.0, {maxDecimalPoints: 2})
})

</script>

<template>
  <main>
    <h2>
      <span id="small-title">SEARCH AT THE<br /></span>
      <span id="large-title">TABLE</span>
    </h2>
    <p>
      An actual play web app that allows you to search for key terms across a
      whole season of Friends at the Table transcripts at once!
    </p>
    <p>
      Just select which season of FaTT you want to search through and enter
      your search term.<br>You can also click the episode title to go
      straight to the transcript itself. You can view all completed
      transcripts by
      <a id="transcripts-link"
        href="https://docs.google.com/spreadsheets/d/1KZHwlSBvHtWStN4vTxOTrpv4Dp9WQrulwMCRocXeYcQ/edit#gid=688483886">clicking
        here</a>
    </p>

    <Form v-model:query="request.query" v-model:kind="request.kind" v-model:seasons="request.seasons" @search="search" />

    <div class="output" aria-live="polite">

      <p class="result-header" v-if="result_err === null && results.episodes.length > 0">
        <b>{{ results.episodes.length }} episodes found so far</b> <br />
        <span id="total-highlights">({{ total_highlights }} results total | took {{ time_taken }})</span>
      </p>

      <p class="result-header" style="color: #ff4747;" v-else-if="result_err !== null">
        <i>error: {{ result_err }}</i>
      </p>

      <p class="result-header" v-else-if="made_request">
        <b>no results >: </b>
      </p>


      <Episode v-for="episode in results.episodes" v-bind="episode" />
      <button id="load-more" @click="load_more" v-show="results.page">Load more</button>

    </div>
  </main>

  <footer>
    <p>
    <p>powered by
      <a href="https://transcriptsatthetable.com" class="link" target="_blank"
        rel="noopener">transcriptsatthetable.com</a> 
        <hr />
    </p>
    originally by
    <a href="https://twitter.com/bryanbakedbean" class="link" target="_blank" rel="noopener">@bryanbakedbean</a>
    /
    currently upkept by
    <a class="link" href="https://cat-girl.gay" target="_blank" rel="noopener">kore signet</a>
    <br />
    </p>
    <p>
      <a class="link" href="docs.html" target="_blank" rel="noopener">api docs</a>
      /
      <a class="link" href="https://github.com/kore-signet/joie" target="_blank" rel="noopener">source code</a>
      /
      <a class="link" href="https://memorious.cat-girl.gay" target="_blank" rel="noopener">library of memorious
        (backup/alternative)</a>
    </p>
    <p>
      <a class="donate" href="https://ko-fi.com/koresignet" target="_blank" rel="noopener">donate/support this project's upkeep and development &lt;3</a>
    </p>
  </footer>
</template>

<style scoped>
main {
  display: block;
  margin: 0 auto;
  width: 70%;
}

h2 {
  font-size: 2.8rem;
  font-family: "Roboto", sans-serif;
  letter-spacing: 7px;
  font-style: italic;
  line-height: 1.6em;
}

#small-title {
  font-weight: 400;
}

#large-title {
  font-weight: 700;
  font-size: 5.7rem;
}

#transcripts-link {
  border-bottom: 2px solid #ffcc00;
  color: #ffcc00;
}

#load-more {
  display: block;

  background: #684cb0;
  font-size: 1.3rem;
  color: white;
  border: none;
  transition: 0.2s;

  min-width: fit-content;
  min-height: 3rem;
  width: 7em;

  margin-bottom: 1em;
}

#load-more:hover {
  background: #ac57ff;
  transition: 0.2s;
  cursor: pointer;
}

.link {
  border-bottom: 2px solid #ffcc00;
  color: #ffcc00;
}

.donate {
  color: #b4e39a;
  font-weight: bold;
}

footer {
  margin-top: 7rem;
}

#total-highlights {
  font-size: 1rem;
}

.result-header {
  margin: 2rem 0;
  font-size: 1.2rem;
}

@media screen and (max-width: 920px) {
  main {
    width: 90%;
  }
}
</style>
