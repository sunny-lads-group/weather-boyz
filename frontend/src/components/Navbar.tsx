import { NavLink } from 'react-router-dom';
import './Navbar.css';
// I haven't used React before this. GPT claims that css files are sometimes stored alongside the component files.
// Vue.js can hold css within the component file, so this seems similar and almost as convenient.
// Unsure what convention we'd like to do, but I'll try to remember to bring it up in the meeting. - Liam

const Navbar = () => {
  return (
    <nav className="navbar">
      <div className="navbar-brand">
        <NavLink to="/">Weather NFT</NavLink>
      </div>
      <div className="navbar-links">
        <NavLink to="/" className={({ isActive }) => 
          isActive ? 'nav-link active' : 'nav-link'
        }>
          Home
        </NavLink>
        <NavLink to="/gallery" className={({ isActive }) => 
          isActive ? 'nav-link active' : 'nav-link'
        }>
          Gallery
        </NavLink>
      </div>
    </nav>
  );
};

export default Navbar; 