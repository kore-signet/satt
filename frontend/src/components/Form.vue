<script setup lang="ts">
const props = defineProps<{
  query: string,
  kind: QueryKind,
  seasons?: string[],
}>()

const emit = defineEmits<{ 
  (e: 'update:query', val: string): void,
  (e: 'update:kind', val: QueryKind): void,
  (e: 'update:seasons', val: string[]): void,
  (e: 'search'): void
}>()

import 'vue-select/dist/vue-select.css'
import MultiSelect from 'primevue/multiselect'

import { QueryKind, type ApiRequest, type ApiResponse } from '../types'
import { computed, reactive, ref } from 'vue'
import * as api from "../api"

const season_options = [
  {
    label: 'Hieron',
    items: [
      {
        label: 'Autumn In Hieron', value: 'autumn-in-hieron'
      },
      {
        label: 'Marielda', value: 'marielda'
      },
      {
        label: 'Winter In Hieron', value: 'winter-in-hieron'
      },
      {
        label: 'Spring In Hieron', value: 'spring-in-hieron'
      }
    ]
  },
  {
    label: 'Divine Cycle',
    items: [
      {
        label: 'Counterweight', value: 'counterweight'
      },
      {
        label: 'Twilight Mirage', value: 'twilight-mirage'
      },
      {
        label: 'Road to PARTIZAN', value: 'road-to-partizan'
      },
      {
        label: 'PARTIZAN', value: 'partizan'
      },
      {
        label: 'Road to PALISADE', value: 'road-to-palisade'
      },
      {
        label: 'PALISADE', value: 'palisade'
      },
    ]
  },
  {
    label: 'Other Seasons',
    items: [
      {
        label: 'Sangfielle', value: 'sangfielle'
      },
      {
        label: 'Perpetua', value: 'perpetua'
      }
    ]
  },
  {
    label: 'Patreon',
    items: [
      { label: 'Patreon (Games)', value: 'patreon-games' },
      { label: 'Patreon (Other)', value: 'patreon-other' }
    ]
  },
  {
    label: 'Others',
    items: [

      {
        label: 'Extras', value: 'extras'
      },
      {
        label: 'Media Club Plus', value: 'media-club-plus'
      },
      {
        label: 'Other', value: 'other'
      }
    ]
  }
]

const query = computed({
  get() {
    return props.query
  },
  set(value: string) {
    emit('update:query', value)
  }
})

const kind = computed({
  get() {
    return props.kind
  },
  set(value: QueryKind) {
    emit('update:kind', value)
  }
})

const seasons = computed({
  get() {
    return props.seasons || []
  },
  set(value: string[]) {
    emit('update:seasons', value)
  }
})
// query: "",
//   kind: QueryKind.PHRASE,
//   seasons: [],
//   highlight: true,
//   page: null

async function search() {
  emit('search')
}

</script>

<template>
  <form>
    <div class="row row-wrapping" style="margin-top: 2rem">
      <MultiSelect id="season-select" filter v-model="seasons" :options="season_options" option-value="value"
        option-label="label" option-group-label="label" option-group-children="items"
        placeholder="seasons to search through">

      </MultiSelect>
      <select id="query-kind" v-model="kind">
        <option value="phrase" selected>Exact match</option>
        <option value="advanced">Advanced (boolean operators!)</option>
      </select>
      <p id="query-help"><i><a class="link" href="query.html" target="_blank" rel="noopener">?</a></i></p>
    </div>
    <div class="row" style="margin-top: 1.5rem">
      <input type="text" name="search_bar" id="search-bar" placeholder="Input your search term here"
        v-model="query" />
      <button @click.prevent="search" type="submit" id="submit">Search!</button>
    </div>
  </form>
</template>

<style scoped>
#season-select {
  width: fit-content;
  max-width: 100%;
}

#search-bar {
  width: 75%;
  max-width: 100%;

  background: #363636;
  color: white;

  font-size: 1.1rem;
  border: 3px solid transparent;
  outline: none;
  margin-left: 2%;

  padding: 1em 0.4em;

  flex-grow: 1;
}

.row-wrapping {
  flex-wrap: wrap;
  gap: 0.4em 0;
}

.row {
  display: flex;
  flex-direction: row;

  justify-content: center;
  align-items: stretch;
  width: 100%;
}

#query-kind {
  max-width: fit-content;
  width: fit-content;

  background: #363636;
  color: white;

  font-size: 1.1rem;
  border: 3px solid transparent;
  outline: none;
  margin-left: 2%;

  padding: 1em 0.4em;
}

#query-help {
  margin-left: 1rem;
}


#submit {
  display: block;

  background: #684cb0;
  font-size: 1.3rem;
  color: white;
  border: none;
  transition: 0.2s;

  min-width: fit-content;
  width: 15%;
}

#submit:hover {
  background: #ac57ff;
  transition: 0.2s;
  cursor: pointer;
}

.link {
  border-bottom: 2px solid #ffcc00;
  color: #ffcc00;
}
</style>
