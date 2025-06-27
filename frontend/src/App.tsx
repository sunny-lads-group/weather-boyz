import { WalletProvider } from './context/WalletContext';
import { AuthProvider } from './context/AuthContext';
import Layout from './components/layout/Layout';
import { BrowserRouter as Router } from 'react-router-dom';

function App() {
  return (
    <AuthProvider>
      <WalletProvider>
        <Router>
          <Layout />
        </Router>
      </WalletProvider>
    </AuthProvider>
  );
}

export default App;