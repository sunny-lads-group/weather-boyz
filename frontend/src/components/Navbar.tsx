import { NavLink } from 'react-router-dom';

const Navbar = () => {
  const navLinkStyles = ({ isActive }: { isActive: boolean }) => 
    `px-3 py-2 rounded-md text-sm font-medium ${
      isActive 
        ? 'bg-gray-900 text-white' 
        : 'text-gray-300 hover:bg-gray-700 hover:text-white'
    }`;

  return (
    <nav className="bg-orange-500 px-4 py-3">
      <div className="flex items-center justify-between">
        <div className="flex items-center">
          <NavLink to="/" className="text-white font-bold text-xl">
            WEATHER BOYZ
          </NavLink>
        </div>
        
        <div className="flex space-x-4">
          <NavLink to="/" className={navLinkStyles}>
            Home
          </NavLink>
          <NavLink to="/policies" className={navLinkStyles}>
            MyPolicies
          </NavLink>
          <NavLink to="/login" className={navLinkStyles}>
            Login
          </NavLink>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;