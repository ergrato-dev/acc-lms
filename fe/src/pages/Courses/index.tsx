import { useTranslation } from 'react-i18next';

/**
 * Courses list page placeholder
 */
function CoursesPage() {
  const { t } = useTranslation();

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold">
        {t('courses:title', 'Catálogo de Cursos')}
      </h1>
      <p className="text-muted-foreground mt-4">
        Página de cursos - En construcción
      </p>
    </div>
  );
}

export default CoursesPage;
