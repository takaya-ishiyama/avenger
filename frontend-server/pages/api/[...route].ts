import { Hono } from "hono";
import { handle } from "hono/vercel";
import init from "../../../frontend/out/frontend.js";

export const config = {
  runtime: "edge",
};

export const initFn = async () => {
  await init();
};
const app = new Hono().basePath("/api");

app.get("/hello", (c) => {
  return c.json({
    message: "Hello from Hono!",
  });
});

export default handle(app);
