import { Button } from '@/components/ui';
import { ROUTES } from '@/utils/constants';
import { Link } from 'react-router-dom';

export function NotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center px-4 text-center">
      <h1 className="text-primary text-9xl font-bold">404</h1>
      <h2 className="mt-4 text-2xl font-semibold">Página no encontrada</h2>
      <p className="text-muted-foreground mt-2">
        Lo sentimos, la página que buscas no existe o ha sido movida.
      </p>
      <Button className="mt-8" asChild>
        <Link to={ROUTES.HOME}>Volver al Inicio</Link>
      </Button>
    </div>
  );
}

export default NotFound;
