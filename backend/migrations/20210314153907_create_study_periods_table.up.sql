CREATE TABLE study_periods (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  year INTEGER NOT NULL,
  period INTEGER NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (year, period)
);

SELECT manage_updated_at('study_periods');
