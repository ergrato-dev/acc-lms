import { useTranslation } from 'react-i18next';
import { useParams } from 'react-router-dom';

/**
 * Course detail page placeholder
 */
function CourseDetailPage() {
  const { slug } = useParams<{ slug: string }>();
  const { t } = useTranslation();

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold">
        {t('courses:detail.title', 'Detalle del Curso')}
      </h1>
      <p className="text-muted-foreground mt-4">
        Curso: {slug} - En construcción
      </p>
    </div>
  );
}

export default CourseDetailPage;
