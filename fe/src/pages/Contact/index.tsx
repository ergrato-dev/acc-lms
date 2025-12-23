import { useTranslation } from 'react-i18next';

/**
 * Contact page placeholder
 */
function ContactPage() {
  const { t } = useTranslation();

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold">
        {t('common:nav.contact', 'Contacto')}
      </h1>
      <p className="text-muted-foreground mt-4">
        Página de contacto - En construcción
      </p>
    </div>
  );
}

export default ContactPage;
