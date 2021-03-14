CREATE TABLE study_years (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  year INTEGER NOT NULL UNIQUE,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT manage_updated_at('study_years');
