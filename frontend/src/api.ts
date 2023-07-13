import { reactive } from "vue";
import { QueryKind, type ApiRequest, type ApiResponse, type ApiResult } from "./types";

const api_url: string = process.env.NODE_ENV == "development" ? "http://localhost:8080/api/search?" : "/api/search?";


export async function search(
    request: Partial<ApiRequest>
): Promise<ApiResult> {
    let req: Record<string, string> = {
        query: request.query || "",
        kind: request.kind || QueryKind.PHRASE,
        seasons:  request.seasons ? request.seasons.join(",") : "",
        highlight: (request.highlight || true).toString(),
    };

    if (request.page) {
        req["page"] = request.page;
    }

    let res = await fetch(
        api_url + new URLSearchParams(req),
        { mode: "cors" }
    );

    let res_json = await res.json();

    if (!res.ok) {
        return { ok: false, msg: res_json["msg"] || res.statusText }
    } else {
        return { ok: true, value: res_json as ApiResponse };

    }
}

export const all_seasons = [
    "autumn-in-hieron",
    "marielda",
    "winter-in-hieron",
    "spring-in-hieron",
    "counterweight",
    "twilight-mirage",
    "road-to-partizan",
    "partizan",
    "road-to-palisade",
    "palisade",
    "sangfielle",
    "extras",
    "patreon",
    "other",
].sort();
