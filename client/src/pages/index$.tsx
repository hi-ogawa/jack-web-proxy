import { NodeEditor } from "../components/node-editor";

export default function Page() {
  return (
    <div className="flex justify-center items-center">
      <div className="w-xl h-xl">
        <NodeEditor />
      </div>
    </div>
  );
}
