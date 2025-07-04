import { createContext, useContext, useState, useEffect, useRef, type ReactNode } from 'react';
import { isTokenExpired } from '../utils/auth';
import { useNotifications } from './NotificationContext';
import { validateTokenWithServer } from '../services/authService';

interface User {
  id: string;
  email: string;
  name?: string;
}

interface AuthContextType {
  isAuthenticated: boolean;
  user: User | null;
  login: (token: string, userData?: User) => void;
  logout: () => void;
  loading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const { addNotification } = useNotifications();
  const validationIntervalRef = useRef<NodeJS.Timeout | null>(null);
  const activityTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    // Check for existing token and user data on app load
    const token = localStorage.getItem('authToken');
    const userData = localStorage.getItem('userData');
    
    if (token && userData) {
      try {
        // Check if token is expired
        if (isTokenExpired(token)) {
          console.log('Token expired, logging out user');
          localStorage.removeItem('authToken');
          localStorage.removeItem('userData');
          setIsAuthenticated(false);
          setUser(null);
          addNotification({
            type: 'warning',
            title: 'Session Expired',
            message: 'Your login session has expired. Please sign in again.',
            duration: 6000
          });
        } else {
          const parsedUser = JSON.parse(userData) as User;
          setIsAuthenticated(true);
          setUser(parsedUser);
          
          // Start token validation for existing session
          setTimeout(() => {
            startTokenValidation();
          }, 1000);
        }
      } catch (error) {
        console.error('Error parsing user data from localStorage:', error);
        // Clear invalid data
        localStorage.removeItem('authToken');
        localStorage.removeItem('userData');
        setIsAuthenticated(false);
        setUser(null);
      }
    }
    setLoading(false);
  }, [addNotification]);

  const handleTokenExpiry = () => {
    logout();
    addNotification({
      type: 'warning',
      title: 'Session Expired',
      message: 'Your login session has expired. Please sign in again.',
      duration: 6000
    });
  };

  const validateToken = async () => {
    const token = localStorage.getItem('authToken');
    console.log('🔍 Token validation triggered', { hasToken: !!token, isAuthenticated });
    
    if (!token || !isAuthenticated) {
      console.log('❌ No token or not authenticated, skipping validation');
      return;
    }

    // First check client-side expiry (fast check)
    if (isTokenExpired(token)) {
      console.log('⏰ Client-side token expiry detected');
      handleTokenExpiry();
      return;
    }

    console.log('🌐 Checking token with server...');
    // Then check with server
    const isValid = await validateTokenWithServer();
    console.log('📡 Server validation response:', { isValid });
    
    if (!isValid) {
      console.log('❌ Server says token is invalid, logging out');
      handleTokenExpiry();
    } else {
      console.log('✅ Token is valid');
    }
  };

  const scheduleActivityValidation = () => {
    if (activityTimeoutRef.current) {
      clearTimeout(activityTimeoutRef.current);
    }
    
    console.log('⏱️ Scheduling activity-based token validation...');
    activityTimeoutRef.current = setTimeout(() => {
      console.log('🎯 Activity-based validation triggered');
      validateToken();
    }, 1000); // Validate 1 second after activity
  };

  const startTokenValidation = () => {
    console.log('🚀 Starting token validation system...');
    
    // Periodic validation every 3 minutes
    if (validationIntervalRef.current) {
      clearInterval(validationIntervalRef.current);
    }
    
    validationIntervalRef.current = setInterval(() => {
      console.log('⏰ Periodic validation triggered (3 minutes elapsed)');
      validateToken();
    }, 180000); // 3 minutes

    console.log('📅 Periodic validation timer set (every 3 minutes)');

    // Activity-based validation
    const handleActivity = () => scheduleActivityValidation();
    
    window.addEventListener('click', handleActivity);
    window.addEventListener('keydown', handleActivity);
    window.addEventListener('focus', handleActivity);

    // Cleanup function
    return () => {
      window.removeEventListener('click', handleActivity);
      window.removeEventListener('keydown', handleActivity);
      window.removeEventListener('focus', handleActivity);
    };
  };

  const login = (token: string, userData?: User) => {
    localStorage.setItem('authToken', token);
    if (userData) {
      localStorage.setItem('userData', JSON.stringify(userData));
    }
    setIsAuthenticated(true);
    setUser(userData || null);
    
    // Start token validation after login
    setTimeout(() => {
      startTokenValidation();
    }, 1000);
  };

  const logout = () => {
    localStorage.removeItem('authToken');
    localStorage.removeItem('userData');
    localStorage.removeItem('walletAddress');
    setIsAuthenticated(false);
    setUser(null);
    
    // Clear validation timers
    if (validationIntervalRef.current) {
      clearInterval(validationIntervalRef.current);
      validationIntervalRef.current = null;
    }
    if (activityTimeoutRef.current) {
      clearTimeout(activityTimeoutRef.current);
      activityTimeoutRef.current = null;
    }
    
    // Notify wallet context to clear its state
    window.dispatchEvent(new CustomEvent('walletDisconnect'));
  };

  const value = {
    isAuthenticated,
    user,
    login,
    logout,
    loading
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}; 