import React, { useState } from "react";
import ReactFlow, {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Connection,
  Edge,
  EdgeChange,
  Handle,
  Node,
  NodeChange,
  NodeProps,
  Position,
} from "react-flow-renderer";

// https://reactflow.dev/docs/getting-started/core-concepts/

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
    (changes: NodeChange[]) => setNodes((nds) => applyNodeChanges(changes, nds)),
    [setNodes]
  );
  const onEdgesChange = React.useCallback(
    (changes: EdgeChange[]) => setEdges((eds) => applyEdgeChanges(changes, eds)),
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
    />
  );
}

const NODE_TYPES = {
  custom: CustomNode,
};

interface NodeData {
  name: string;
}

function CustomNode(props: NodeProps<NodeData>) {
  return <>
    <Handle type="target" id="z" position={Position.Left} isConnectable>
      <div className="border">
        z
      </div>
    </Handle>
    <div className="p-2 border">{props.data.name}</div>
    <Handle type="source" id="x" position={Position.Right} isConnectable style={{ top: 2 }}>x</Handle>
    <Handle type="source" id="y" position={Position.Right} isConnectable style={{ top: "auto", bottom: 2 }} />
  </>;
}
