import { NavLink } from 'react-router-dom';
import { useWallet } from '../../context/WalletContext';
import WalletConnect from '../WalletConnect';

const Navbar = () => {
  const { isConnected } = useWallet();

  return (
    <nav className="bg-orange-500 px-4 py-3 shadow-lg">
      <div className="container mx-auto flex items-center justify-between">
        <NavLink to="/" className="text-white font-bold text-xl">
          WEATHER BOYZ
        </NavLink>
        
        <div className="flex items-center space-x-6">
          <NavLink
            to="/"
            className={({ isActive }) =>
              `text-white hover:text-orange-200 ${isActive ? 'font-bold' : ''}`
            }
          >
            Home
          </NavLink>
          
          {isConnected && (
            <>
              <NavLink
                to="/available-policies"
                className={({ isActive }) =>
                  `text-white hover:text-orange-200 ${isActive ? 'font-bold' : ''}`
                }
              >
                Available Policies
              </NavLink>
              <NavLink
                to="/my-policies"
                className={({ isActive }) =>
                  `text-white hover:text-orange-200 ${isActive ? 'font-bold' : ''}`
                }
              >
                My Policies
              </NavLink>
            </>
          )}
          
          <WalletConnect />
        </div>
      </div>
    </nav>
  );
};

export default Navbar;