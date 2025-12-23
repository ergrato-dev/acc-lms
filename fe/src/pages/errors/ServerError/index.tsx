import { Button } from '@/components/ui';
import { ROUTES } from '@/utils/constants';
import { Link } from 'react-router-dom';

export function ServerError() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center px-4 text-center">
      <h1 className="text-destructive text-9xl font-bold">500</h1>
      <h2 className="mt-4 text-2xl font-semibold">Error del Servidor</h2>
      <p className="text-muted-foreground mt-2">
        Lo sentimos, algo salió mal. Por favor intenta de nuevo más tarde.
      </p>
      <Button className="mt-8" asChild>
        <Link to={ROUTES.HOME}>Volver al Inicio</Link>
      </Button>
    </div>
  );
}

export default ServerError;
