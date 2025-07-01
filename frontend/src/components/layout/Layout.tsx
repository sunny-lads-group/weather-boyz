import { Routes, Route, Navigate } from 'react-router-dom';
import Navbar from './Navbar';
import Home from '../../pages/Home';
import AvailablePolicies from '../../pages/AvailablePolicies';
import MyPolicies from '../../pages/MyPolicies';
import Login from '../../pages/Login';
import Register from '../../pages/Register';
import { useWallet } from '../../context/WalletContext';
import { useAuth } from '../../context/AuthContext';

const Layout = () => {
  const { isConnected } = useWallet();
  const { isAuthenticated, loading } = useAuth();

  // Show loading spinner while checking authentication
  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-100">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-indigo-600"></div>
      </div>
    );
  }

  // If not authenticated, show login/register pages
  if (!isAuthenticated) {
    return (
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="*" element={<Navigate to="/login" replace />} />
      </Routes>
    );
  }

  // If authenticated, show the main app
  return (
    <div className="min-h-screen bg-gray-100">
      <Navbar />
      <main className="container mx-auto px-4 py-8">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route
            path="/available-policies"
            element={isConnected ? <AvailablePolicies /> : <Navigate to="/" />}
          />
          <Route
            path="/my-policies"
            element={isConnected ? <MyPolicies /> : <Navigate to="/" />}
          />
        </Routes>
      </main>
    </div>
  );
};

export default Layout;