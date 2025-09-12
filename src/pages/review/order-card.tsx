import { Card, CardContent } from "@/components/ui/card";
import {
  compareDateTimes,
  dateFormatter,
  excelTimeToHHMM,
  timeFormatter,
} from "@/features/time";
import { usePrecision } from "@/hooks";
import { AlarmClockCheck, CookingPot, DoorOpen } from "lucide-react";
import { Denied } from "@/features/denied";
import { PreparedRow } from "./api";
import { TimeDisplay } from "./time-display";

type Props = {
  row: PreparedRow;
  approved: boolean;
};

export function OrderCard(props: Props) {
  const { approved, row } = props;
  const [precision] = usePrecision();

  const inTime = new Date(row.suggestedIn!);
  const outTime = new Date(row.suggestedOut!);

  return (
    <Card className="flex-1 hover:shadow-lg transition-shadow rounded-lg overflow-hidden">
      <CardContent className="flex">
        <div className="flex flex-1 gap-4 items-start">
          <div className="text-xs text-gray-400 whitespace-nowrap">
            {dateFormatter.format(new Date(row.order.datetime))}
          </div>
          <div className="flex flex-col">
            <div className="text-sm font-semibold truncate">
              {row.order.employee}
            </div>
            <div className="text-xs text-gray-500 truncate">
              {row.order.client}
            </div>
          </div>
        </div>
        <div className="relative flex flex-1 justify-between">
          <TimeDisplay
            icon={<CookingPot />}
            time={excelTimeToHHMM(row.order.ready)}
            label="Kitchen Ready"
          />
          <TimeDisplay
            color={compareDateTimes(
              precision,
              row.order.ready,
              inTime,
            )}
            icon={<AlarmClockCheck />}
            time={timeFormatter.format(inTime)}
            label="Clock In"
          />
          <TimeDisplay
            icon={<DoorOpen />}
            time={timeFormatter.format(outTime)}
            label="Clock Out"
          />

          {!approved && <Denied />}
        </div>
      </CardContent>
    </Card>
  );
}
