import { Routes, Route, Navigate } from 'react-router-dom';
import Navbar from './Navbar';
import Home from '../../pages/Home';
import AvailablePolicies from '../../pages/AvailablePolicies';
import MyPolicies from '../../pages/MyPolicies';
import { useWallet } from '../../context/WalletContext';

const Layout = () => {
  const { isConnected } = useWallet();

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