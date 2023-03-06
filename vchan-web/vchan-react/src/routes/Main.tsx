import { CookieIcon } from "@radix-ui/react-icons";
import { VcrButton } from "../components/VcrButton";

export function Index() {
  return (
    <article className="w-full h-full justify-center text-white px-5">
      <h1 className="text-5xl">拟真信道</h1>
      <br />
      <q className="text-4xl">一切固结的，都已融化在空气中。</q>
      <VcrButton aria-label="btn1">按钮1</VcrButton>
      <VcrButton aria-label="btn1">按钮2</VcrButton>
      <VcrButton aria-label="btn1">
        <CookieIcon />
      </VcrButton>
      <VcrButton aria-label="btn1">
        按钮3
        <CookieIcon />
      </VcrButton>
    </article>
  );
}