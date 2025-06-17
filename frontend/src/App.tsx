import { WalletProvider } from './context/WalletContext';
import Layout from './components/layout/Layout';
import { BrowserRouter as Router } from 'react-router-dom';

function App() {
  return (
    <WalletProvider>
      <Router>
        <Layout />
      </Router>
    </WalletProvider>
  );
}

export default App;