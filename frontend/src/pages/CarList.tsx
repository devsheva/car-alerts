import { Eraser, Eye } from 'lucide-react'
import * as R from 'ramda'
import { useCars } from '../hooks/useCars'

export default function CarList() {
  const { data: cars, isLoading } = useCars()

  console.debug({
    cars,
    isLoading,
  })

  return (
    <div className="p-4 space-y-4">
      <div className="flex items-center justify-between px-4">
        <h2 className="text-xl font-bold">My Cars</h2>
        <button
          className="inline-flex items-center gap-2 text-red-400 hover:text-red-300"
          onClick={() => console.warn('call reset api.')}
        >
          <Eraser />
        </button>
      </div>

      {R.map(
        (car) => (
          <div
            key={car.plate}
            className="border border-white/10 rounded-lg p-4 shadow-sm"
          >
            <div className="flex justify-between items-start mb-2">
              <h3 className="text-lg font-semibold">{car.brand}</h3>
              <span className="text-sm">{car.plate}</span>
              <button onClick={() => console.warn('go to detail')}>
                <Eye />
              </button>
            </div>

            <div className="text-sm text-white/70 space-y-1">
              <p>
                <span className="text-white/50">Owner: {car.owner}</span>
              </p>
              <p>
                <span className="text-white/50">
                  Last Revision: {car.last_revision}
                </span>
              </p>
              <p>
                <span className="text-white/50">
                  Last Road Tax: {car.last_road_tax}
                </span>
              </p>
            </div>
          </div>
        ),
        cars,
      )}
    </div>
  )
}
