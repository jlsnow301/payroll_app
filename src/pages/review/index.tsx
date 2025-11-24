import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../../components/ui/card.tsx";
import { PurpleBg, Stars } from "../../features/animated-bg/stars.tsx";
import { useSubmissionResult } from "../../hooks.ts";

export function ReviewPage() {
  const [results] = useSubmissionResult();

  return (
    <PurpleBg>
      <Stars />
      <Card className="w-7/8 h-7/8 z-10 opacity-90 select-none">
        <CardHeader>
          <CardTitle>Process Results</CardTitle>
          <CardDescription>
            Here are the results of your submission.
          </CardDescription>
        </CardHeader>

        <CardContent className="flex flex-col gap-4 text-lg">
          <span>Expanded: {results?.expanded}</span>
          <span>Matched: {results?.matched}</span>
          <span>Skipped: {results?.skipped}</span>
          <span>Total: {results?.total}</span>
          <span>Top Used: {results?.top_used} ({results?.top_used_count})</span>
          <span>
            Most Punctual: {results?.punctual} ({results?.punctual_avg})
          </span>
        </CardContent>
      </Card>
    </PurpleBg>
  );
}
