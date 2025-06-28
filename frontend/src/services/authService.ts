export const validateTokenWithServer = async (): Promise<boolean> => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    console.log('🚫 No token found in localStorage');
    return false;
  }

  try {
    console.log('📞 Making request to /tokenvalid endpoint...');
    const response = await fetch('http://localhost:3000/tokenvalid/', {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      }
    });

    console.log('📡 Server response:', { 
      status: response.status, 
      statusText: response.statusText, 
      ok: response.ok 
    });

    return response.ok;
  } catch (error) {
    console.warn('🌐 Token validation failed due to network error:', error);
    // Don't treat network errors as invalid tokens
    return true;
  }
};