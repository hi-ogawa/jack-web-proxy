import { NodeEditor } from "../components/node-editor";

export default function Page() {
  return (
    <div className="h-full flex justify-center items-center p-4 bg-gray-50">
      <div className="w-4xl h-4xl border bg-white">
        <NodeEditor />
      </div>
    </div>
  );
}
