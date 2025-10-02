-- Criação dos bancos
CREATE DATABASE api_go;
CREATE DATABASE api_rust;
CREATE DATABASE api_java;
CREATE DATABASE api_csharp;

-- Criar tabela tokens em cada DB
\connect api_go;
CREATE TABLE tokens (
  id TEXT PRIMARY KEY,
  date DATE NOT NULL,
  time TIME NOT NULL,
  batch TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

\connect api_rust;
CREATE TABLE tokens (
  id TEXT PRIMARY KEY,
  date DATE NOT NULL,
  time TIME NOT NULL,
  batch TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

\connect api_java;
CREATE TABLE tokens (
  id TEXT PRIMARY KEY,
  date DATE NOT NULL,
  time TIME NOT NULL,
  batch TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

\connect api_csharp;
CREATE TABLE tokens (
  id TEXT PRIMARY KEY,
  date DATE NOT NULL,
  time TIME NOT NULL,
  batch TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
