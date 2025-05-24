import { createRoot } from 'react-dom/client';
import { createInertiaApp } from '@inertiajs/react';
import './styles.css';

createInertiaApp({
  resolve: name => {
    const pages = import.meta.glob('./views/pages/**/*.tsx', { eager: true });
    return pages[`./views/pages/${name}.tsx`];
  },
  setup({ el, App, props }) {
    createRoot(el).render(<App {...props} />);
  },
});
