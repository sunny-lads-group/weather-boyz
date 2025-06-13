import { useState } from 'react'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import './App.css'
import WalletConnect from './components/WalletConnect'
import Navbar from './components/Navbar'
import Gallery from './pages/Gallery'

function App() {
  const [connectedAddress, setConnectedAddress] = useState<string>('')
  const [message, setMessage] = useState('')

  const handleWalletConnect = (address: string) => {
    setConnectedAddress(address)
    console.log('Wallet connected:', address)
  }

  return (
    <Router>
      <div className="App">
        <Navbar />
        <main className="main-content">
          <Routes>
            <Route path="/" element={
              <>
                <h1>Weather NFT Minter</h1>
                <WalletConnect onConnect={handleWalletConnect} />
                {connectedAddress && (
                  <div className="connected-status">
                    <p>Ready to mint your weather NFT!</p>
                  </div>
                )}
              </>
            } />
            <Route path="/gallery" element={<Gallery />} />
          </Routes>
        </main>
      </div>
    </Router>
  )
}

export default App