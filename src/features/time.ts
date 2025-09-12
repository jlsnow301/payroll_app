export const dateFormatter = new Intl.DateTimeFormat("en-us", {
  month: "numeric",
  day: "numeric",
});

export const timeFormatter = new Intl.DateTimeFormat("en-us", {
  hour: "2-digit",
  minute: "2-digit",
  hour12: false,
  timeZone: "America/Los_Angeles",
});

export function excelTimeToHHMM(excelTime: number): string {
  const totalMinutes = Math.round(excelTime * 24 * 60);

  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;

  return `${hours.toString().padStart(2, "0")}:${
    minutes.toString().padStart(2, "0")
  }`;
}

export enum Threshold {
  Red,
  Yellow,
  Green,
  Gray, //Default
}

/** Excel dates are sent without date information so this sets it to a Date obj
that is from the second argument */
function getExcelDate(excelDt: number, setTo: Date): Date {
  const timeFraction = excelDt % 1;
  const totalMinutes = timeFraction * 24 * 60;
  const hours = Math.floor(totalMinutes / 60);
  const minutes = Math.floor(totalMinutes % 60);
  const seconds = Math.floor((totalMinutes % 1) * 60);

  const target = new Date(
    setTo.getFullYear(),
    setTo.getMonth(),
    setTo.getDate(),
    hours,
    minutes,
    seconds,
  );

  return target;
}

export function compareDateTimes(
  maxHours: number,
  excelDt: number,
  actual: Date,
): Threshold {
  const target = getExcelDate(excelDt, actual);
  const diffMs = actual.getTime() - target.getTime();
  const diffHours = diffMs / (1000 * 60 * 60);

  if (diffHours <= 0) {
    const absDiffHours = Math.abs(diffHours);
    const greenThreshold = maxHours * 0.5;
    const yellowThreshold = maxHours;

    if (absDiffHours <= greenThreshold) {
      return Threshold.Green;
    } else if (absDiffHours <= yellowThreshold) {
      return Threshold.Yellow;
    } else {
      return Threshold.Red;
    }
  } else {
    const greenThreshold = maxHours * 0.25;
    const yellowThreshold = maxHours * 0.5;

    if (diffHours <= greenThreshold) {
      return Threshold.Green;
    } else if (diffHours <= yellowThreshold) {
      return Threshold.Yellow;
    } else {
      return Threshold.Red;
    }
  }
}
