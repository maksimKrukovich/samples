/* eslint-disable prettier/prettier */
import 'react-toastify/dist/ReactToastify.css';
import 'reflect-metadata';
import './App.styles.css';

import { Sepolia } from '@thirdweb-dev/chains';
import { metamaskWallet, ThirdwebProvider } from '@thirdweb-dev/react';
import { FC } from 'react';
import { QueryClient, QueryClientProvider } from 'react-query';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import { RouteConstants } from './constants';
import { HomePage } from './pages';

const queryClient = new QueryClient();

export const App: FC = () => {
  return (
    <QueryClientProvider client={queryClient}>
      {/* <ReactQueryDevtools initialIsOpen={true} /> */}
      <ThirdwebProvider activeChain={Sepolia} supportedWallets={[metamaskWallet()]}>
        <Router>
          <Routes>
            <Route path={RouteConstants.Home} element={<HomePage />}></Route>
            <Route path={RouteConstants.Uni} element={<p>Uni</p>}></Route>
          </Routes>
        </Router>
      </ThirdwebProvider>
    </QueryClientProvider>
  );
};
