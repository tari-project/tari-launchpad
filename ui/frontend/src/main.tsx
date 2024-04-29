import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles.css';
import queryClient from './api/queryClient';
import { QueryClientProvider } from '@tanstack/react-query';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </React.StrictMode>
);
