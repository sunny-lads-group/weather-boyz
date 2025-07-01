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