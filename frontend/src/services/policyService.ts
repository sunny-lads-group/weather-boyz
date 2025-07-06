export interface CreatePolicyRequest {
  policy_template_id?: number;
  policy_name: string;
  policy_type: string;
  location_latitude: number;
  location_longitude: number;
  location_h3_index?: string;
  location_name?: string;
  coverage_amount: number;
  premium_amount: number;
  currency?: string;
  start_date: string;
  end_date: string;
  weather_station_id?: string;
  smart_contract_address?: string;
  purchase_transaction_hash: string;
}

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:6969';

export const fetchPolicyTemplates = async () => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    throw new Error('No authentication token found');
  }

  try {
    const response = await fetch(`${API_URL}/policy-templates`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error(
        `Failed to fetch policy templates: ${response.status} ${response.statusText}`
      );
    }

    return await response.json();
  } catch (error) {
    console.error('Error fetching policy templates:', error);
    throw error;
  }
};

export const createPolicy = async (policyData: CreatePolicyRequest) => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    throw new Error('No authentication token found');
  }

  try {
    const response = await fetch(`${API_URL}/policies`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(policyData),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(
        `Failed to create policy: ${response.status} ${response.statusText} - ${errorText}`
      );
    }

    return await response.json();
  } catch (error) {
    console.error('Error creating policy:', error);
    throw error;
  }
};

export const fetchUserPolicies = async () => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    throw new Error('No authentication token found');
  }

  try {
    const response = await fetch(`${API_URL}/policies`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error(
        `Failed to fetch user policies: ${response.status} ${response.statusText}`
      );
    }

    return await response.json();
  } catch (error) {
    console.error('Error fetching user policies:', error);
    throw error;
  }
};
