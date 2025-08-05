import { LoaderCircle } from "lucide-react";

export default function Spinner() {
  return (
    <div className="flex items-center justify-center min-h-screen">
      <LoaderCircle className="animate-spin" size={16} />
    </div>
  );
}
