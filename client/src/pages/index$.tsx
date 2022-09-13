import { NodeEditor } from "../components/node-editor";
import { NodeEditorFlume } from "../components/node-editor-flume";

export default function Page() {
  return (
    <div className="flex justify-center items-center">
      <div className="w-xl h-xl">
        {/* <NodeEditor /> */}
        <NodeEditorFlume />
      </div>
    </div>
  );
}
