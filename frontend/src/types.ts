export type EpisodeData = {
    curiosity_id: number,
    slug: string,
    season: string,
    title: string,
    docs_id: string,
    highlights: { text: string, highlighted: boolean }[][]
}

export enum QueryKind  {
    PHRASE = "phrase",
    ADVANCED = "advanced"
}

export interface ApiRequest  {
    query: string,
    kind: QueryKind,
    seasons?: string[],
    highlight: boolean,
    page?: string | null,
    page_size?: number
}

export type ApiResponse = {
    query_time: number,
    next_page: string | null,
    episodes: EpisodeData[]
}

export type ApiError = {
    err: true,
    msg: string
}

export type ApiResult = 
    | { ok: true; value: ApiResponse }
    | { ok: false; msg: string }
    