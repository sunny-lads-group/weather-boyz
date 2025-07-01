import { WalletProvider } from './context/WalletContext';
import { AuthProvider } from './context/AuthContext';
import { NotificationProvider } from './context/NotificationContext';
import Layout from './components/layout/Layout';
import NotificationContainer from './components/ui/NotificationContainer';
import { BrowserRouter as Router } from 'react-router-dom';

function App() {
  return (
    <NotificationProvider>
      <AuthProvider>
        <WalletProvider>
          <Router>
            <Layout />
            <NotificationContainer />
          </Router>
        </WalletProvider>
      </AuthProvider>
    </NotificationProvider>
  );
}

export default App;