import { LoaderCircle } from 'lucide-react'

export default function Loading() {
  return (
    <div className="flex items-center justify-center min-h-screen">
      <LoaderCircle className="animate-spin" size={'2.5rem'} />
    </div>
  )
}
