import { useCallback, useEffect, useState } from "react";
import dynamic from "next/dynamic";
import init from "../out/frontend.js";
import { NextPage } from "next";

export default function Home() {
  // const WasmSample = dynamic(
  //   {
  //     loader: async () => {
  //       const initWasm = await import("../out/frontend_bg.wasm.js");

  //       console.log(initWasm);
  //       return () => <button onClick={() => {}}>実行する</button>;
  //     },
  //   },
  //   { ssr: false }
  // );
  console.log(init());

  return <p>aaaaa</p>;
}
