const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:6969';

export const validateTokenWithServer = async (): Promise<boolean> => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    console.log('🚫 No token found in localStorage');
    return false;
  }

  try {
    console.log('📞 Making request to /tokenvalid endpoint...');
    const response = await fetch(`${API_URL}/tokenvalid/`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    console.log('📡 Server response:', {
      status: response.status,
      statusText: response.statusText,
      ok: response.ok,
    });

    return response.ok;
  } catch (error) {
    console.warn('🌐 Token validation failed due to network error:', error);
    // Don't treat network errors as invalid tokens
    return true;
  }
};

export const updateWalletAddress = async (
  walletAddress: string
): Promise<void> => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    throw new Error('No authentication token found');
  }

  try {
    console.log('📝 Updating wallet address:', walletAddress);
    const response = await fetch(`${API_URL}/user/wallet`, {
      method: 'PUT',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ wallet_address: walletAddress }),
    });

    console.log('📡 Wallet update response:', {
      status: response.status,
      statusText: response.statusText,
      ok: response.ok,
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(
        `Failed to update wallet address: ${response.status} ${response.statusText} - ${errorText}`
      );
    }

    console.log('✅ Wallet address updated successfully');
  } catch (error) {
    console.error('❌ Error updating wallet address:', error);
    throw error;
  }
};
