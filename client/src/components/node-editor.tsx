import React, { useState } from "react";
import ReactFlow, {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Background,
  Connection,
  Edge,
  EdgeChange,
  Handle as Handle_original,
  Node,
  NodeChange,
  NodeProps,
  Position,
} from "react-flow-renderer";
import type { HandleComponentProps } from "react-flow-renderer/dist/esm/components/Handle";

// workaround typescript error "Expression produces a union type that is too complex to represent"
const Handle =
  Handle_original as any as React.ComponentType<HandleComponentProps>;

const initialNodes: Node[] = [
  {
    id: "1",
    type: "input",
    data: { label: "Input Node" },
    position: { x: 250, y: 25 },
  },

  {
    id: "2",
    // you can also pass a React component as a label
    data: { label: <div>Default Node</div> },
    position: { x: 100, y: 125 },
  },
  {
    id: "3",
    type: "output",
    data: { label: "Output Node" },
    position: { x: 250, y: 250 },
  },
  {
    id: "4",
    type: "custom",
    data: { name: "Custom" },
    position: { x: 100, y: 100 },
  },
];

const initialEdges: Edge[] = [
  { id: "e1-2", source: "1", target: "2" },
  { id: "e2-3", source: "2", target: "3", animated: true },
];

export function NodeEditor() {
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);

  const onNodesChange = React.useCallback(
    (changes: NodeChange[]) =>
      setNodes((nds) => applyNodeChanges(changes, nds)),
    [setNodes]
  );
  const onEdgesChange = React.useCallback(
    (changes: EdgeChange[]) =>
      setEdges((eds) => applyEdgeChanges(changes, eds)),
    [setEdges]
  );
  const onConnect = React.useCallback(
    (connection: Connection) => setEdges((eds) => addEdge(connection, eds)),
    [setEdges]
  );

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={NODE_TYPES}
      fitView
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      onConnect={onConnect}
      // https://github.com/wbkd/react-flow/blob/main/src/components/Attribution/index.tsx
      proOptions={{ account: "paid-pro", hideAttribution: true }}
    >
      <Background />
    </ReactFlow>
  );
}

const NODE_TYPES = {
  custom: CustomNode,
};

interface NodeData {
  name: string;
}

// TODO:
// - horizontal layout
// - maybe override `react-flow__handle` to non-absolute position
function CustomNode(props: NodeProps<NodeData>) {
  return (
    <>
      <div className="p-2 border">{props.data.name}</div>
      <Handle
        type="target"
        id="z"
        position={Position.Left}
        isConnectable
        style={{ position: "initial" }}
      >
        <div className="border">z</div>
      </Handle>
      <Handle
        type="source"
        id="x"
        position={Position.Right}
        isConnectable
        style={{ top: 2 }}
      >
        x
      </Handle>
      <Handle
        type="source"
        id="y"
        position={Position.Right}
        isConnectable
        style={{ top: "auto", bottom: 2 }}
      />
    </>
  );
}
