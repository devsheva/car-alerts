import { lazy, Suspense } from 'react'
import './App.css'
import Loading from '@components/Loading'

const CarList = lazy(() => import('pages/CarList'))

const App = () => {
  return (
    <div className="min-h-screen text-white font-sans bg-gradient-to-b from-[#020917] to-[#101725]">
      <header className="p-4 text-center">
        <h1 className="text-2xl  text-purple-400 font-bold">Car Alerts</h1>
      </header>

      <main className="p-4">
        <Suspense fallback={<Loading />}>
          <CarList />
        </Suspense>
      </main>
    </div>
  )
}

export default App
