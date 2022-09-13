type PortFlag =
  | "IS_INPUT"
  | "IS_OUTPUT"
  | "IS_PHYSICAL"
  | "IS_TERMINAL"
  | "CAN_MONITOR";

interface PortInfo {
  id: string;
  client_name: string;
  port_name: string;
  flags: PortFlag[];
}

interface ConnectionInfo {
  source: string;
  destinations: string[];
}

export interface SummaryResponse {
  ports: PortInfo[];
  connections: ConnectionInfo[];
}

export interface ClientData {
  name: string;
  ports: PortInfo;
}
