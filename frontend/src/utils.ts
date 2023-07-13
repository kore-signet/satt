import humanizeDuration from "humanize-duration";

export const duration_humanizer = humanizeDuration.humanizer({
    language: "fatt",
    units: ["m", "s", "ms"],
    languages: {
      fatt: {
        y: (c) => "year" + (c === 1 ? "" : "s"),
        mo: (c) => "month" + (c === 1 ? "" : "s"),
        w: (c) => "week" + (c === 1 ? "" : "s"),
        d: (c) => "day" + (c === 1 ? "" : "s"),
        h: (c) => "hour" + (c === 1 ? "" : "s"),
        m: (c) => "minute" + (c === 1 ? "" : "s"),
        s: (c) => "second" + (c === 1 ? "" : "s"),
        ms: (c) => {
          if (Math.random() > 0.6) {
            return "millisecond" + (c === 1 ? "" : "s");
          } else {
            return "ver'millisecond" + (c === 1 ? "" : "s");
          }
        }
      }
    }
  })

  
  // why do i have to write this in the year of our lord 2023
export function array_unordered_equals(lhs: any[], rhs: any[]): boolean {
    lhs.sort();
    rhs.sort();

    if (lhs.length !== rhs.length) {
        return false;
    }

    for (let i = 0; i < lhs.length; i++) {
        if (lhs[i] !== rhs[i]) {
            return false;
        }
    }

    return true;
}
