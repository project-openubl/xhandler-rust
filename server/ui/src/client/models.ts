export interface Credentials {
  id: number;
  name: string;
  description?: string;
  supplier_ids_applied_to: string[];
  soap?: {
    username_sol: string;
    password_sol: string;
    url_invoice: string;
    url_perception_retention: string;
  };
  rest?: {
    client_id: string;
    client_secret: string;
    url_despatch: string;
  };
}
