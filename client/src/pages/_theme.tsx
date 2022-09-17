import type { ThemeProps } from "vite-plugin-react-pages/theme.doc";
import "virtual:windi.css";

export default function Theme({ loadState, loadedData }: ThemeProps) {
  if (loadState.type !== "loaded") {
    return <pre>loadState = {JSON.stringify(loadState, null, 2)}</pre>;
  }
  const pageData = loadedData[loadState.routePath];
  const Component = pageData?.["main"]?.["default"];
  return Component && <Component />;
}
