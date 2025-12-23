import { useTranslation } from 'react-i18next';

/**
 * About page placeholder
 */
function AboutPage() {
  const { t } = useTranslation();

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold">
        {t('common:nav.about', 'Sobre Nosotros')}
      </h1>
      <p className="text-muted-foreground mt-4">
        Página sobre nosotros - En construcción
      </p>
    </div>
  );
}

export default AboutPage;
