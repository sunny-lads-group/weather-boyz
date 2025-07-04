export interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  precipitation: number;
  feels_like: number;
}

export interface Coordinates {
  lat: number;
  lng: number;
}

export interface LoginFormData {
  email: string;
  password: string;
}

export interface LoginResponse {
  success: boolean;
  message: string;
  token?: string;
}

export interface RegisterFormData {
  name: string;
  email: string;
  password: string;
}

export interface RegisterResponse {
  success: boolean;
  message: string;
  token?: string;
}

export interface PolicyTemplate {
  id: number;
  template_name: string;
  description?: string;
  policy_type: string;
  default_conditions?: any;
  min_coverage_amount: string;
  max_coverage_amount: string;
  base_premium_rate: string;
  is_active?: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface InsurancePolicy {
  id: number;
  user_id: number;
  policy_template_id?: number;
  policy_name: string;
  policy_type: string;
  location_latitude: string;
  location_longitude: string;
  location_h3_index?: string;
  location_name?: string;
  coverage_amount: string;
  premium_amount: string;
  currency?: string;
  start_date: string | number[];
  end_date: string | number[];
  status?: string;
  weather_station_id?: string;
  smart_contract_address?: string;
  purchase_transaction_hash?: string;
  blockchain_verified?: boolean;
  verification_timestamp?: string;
  blockchain_block_number?: number;
  verification_error_message?: string;
  created_at?: string;
  updated_at?: string;
}