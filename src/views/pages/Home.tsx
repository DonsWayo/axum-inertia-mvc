import React from 'react';

interface HomeProps {
  message: string;
}

const Home: React.FC<HomeProps> = ({ message }) => {
  return (
    <div className="min-h-screen bg-gray-100 flex flex-col items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-md max-w-md w-full">
        <h1 className="text-3xl font-bold text-gray-900 mb-4">{message}</h1>
        <p className="text-gray-600">
          This is a simple Axum Inertia MVC application with React and TailwindCSS.
        </p>
        <div className="mt-6 p-4 bg-blue-50 rounded-md">
          <p className="text-blue-800 font-medium">
            Axum + Inertia.js + React + TailwindCSS
          </p>
        </div>
      </div>
    </div>
  );
};

export default Home;
