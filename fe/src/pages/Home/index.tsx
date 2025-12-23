import { Button } from '@/components/ui';
import { ROUTES } from '@/utils/constants';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router-dom';

/**
 * Home page component
 */
function HomePage() {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="from-primary/10 via-background to-secondary/10 relative bg-gradient-to-br">
        <div className="container mx-auto px-4 py-20 md:py-32">
          <div className="mx-auto max-w-3xl text-center">
            <h1 className="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl">
              {t(
                'common:home.hero.title',
                'Aprende nuevas habilidades en línea'
              )}
            </h1>
            <p className="text-muted-foreground mt-6 text-lg md:text-xl">
              {t(
                'common:home.hero.subtitle',
                'Accede a cursos de alta calidad impartidos por expertos. Avanza en tu carrera profesional desde cualquier lugar.'
              )}
            </p>
            <div className="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row">
              <Button size="lg" asChild>
                <Link to={ROUTES.COURSES.LIST}>
                  {t('common:actions.exploreCourses', 'Explorar Cursos')}
                </Link>
              </Button>
              <Button variant="outline" size="lg" asChild>
                <Link to={ROUTES.AUTH.REGISTER}>
                  {t('common:actions.getStarted', 'Comenzar Gratis')}
                </Link>
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20">
        <div className="container mx-auto px-4">
          <h2 className="text-center text-3xl font-bold">
            {t('common:home.features.title', '¿Por qué elegirnos?')}
          </h2>
          <div className="mt-12 grid gap-8 md:grid-cols-3">
            {/* Feature 1 */}
            <div className="text-center">
              <div className="bg-primary/10 mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  className="text-primary"
                >
                  <path d="M22 10v6M2 10l10-5 10 5-10 5z" />
                  <path d="M6 12v5c3 3 9 3 12 0v-5" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold">
                {t('common:home.features.quality.title', 'Cursos de Calidad')}
              </h3>
              <p className="text-muted-foreground mt-2">
                {t(
                  'common:home.features.quality.description',
                  'Contenido actualizado y revisado por expertos en cada área.'
                )}
              </p>
            </div>

            {/* Feature 2 */}
            <div className="text-center">
              <div className="bg-primary/10 mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  className="text-primary"
                >
                  <circle cx="12" cy="8" r="6" />
                  <path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold">
                {t('common:home.features.certificate.title', 'Certificados')}
              </h3>
              <p className="text-muted-foreground mt-2">
                {t(
                  'common:home.features.certificate.description',
                  'Obtén certificados verificables al completar cada curso.'
                )}
              </p>
            </div>

            {/* Feature 3 */}
            <div className="text-center">
              <div className="bg-primary/10 mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  className="text-primary"
                >
                  <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
                  <circle cx="9" cy="7" r="4" />
                  <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
                  <path d="M16 3.13a4 4 0 0 1 0 7.75" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold">
                {t('common:home.features.community.title', 'Comunidad')}
              </h3>
              <p className="text-muted-foreground mt-2">
                {t(
                  'common:home.features.community.description',
                  'Únete a miles de estudiantes y expande tu red profesional.'
                )}
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary text-primary-foreground py-16">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-3xl font-bold">
            {t('common:home.cta.title', '¿Listo para comenzar tu aprendizaje?')}
          </h2>
          <p className="mt-4 text-lg opacity-90">
            {t(
              'common:home.cta.subtitle',
              'Únete hoy y accede a cientos de cursos.'
            )}
          </p>
          <Button variant="secondary" size="lg" className="mt-8" asChild>
            <Link to={ROUTES.AUTH.REGISTER}>
              {t('common:actions.signUp', 'Crear Cuenta')}
            </Link>
          </Button>
        </div>
      </section>
    </div>
  );
}

export default HomePage;
