--
-- PostgreSQL database dump
--

\restrict v8Xosq7eklO5NlHuz8Cre35JGskAzKbNvPtBX0bWgRctz2f4MvnnC7YSSRyspbk

-- Dumped from database version 16.11 (Debian 16.11-1.pgdg13+1)
-- Dumped by pg_dump version 16.11 (Debian 16.11-1.pgdg13+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: categories; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.categories (
    id integer NOT NULL,
    name text NOT NULL,
    sku_initials text NOT NULL,
    description text
);


ALTER TABLE public.categories OWNER TO admin;

--
-- Name: categories_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.categories_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.categories_id_seq OWNER TO admin;

--
-- Name: categories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.categories_id_seq OWNED BY public.categories.id;


--
-- Name: materials; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.materials (
    id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.materials OWNER TO admin;

--
-- Name: materials_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.materials_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.materials_id_seq OWNER TO admin;

--
-- Name: materials_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.materials_id_seq OWNED BY public.materials.id;


--
-- Name: product_materials; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.product_materials (
    product_id integer NOT NULL,
    material_id integer NOT NULL
);


ALTER TABLE public.product_materials OWNER TO admin;

--
-- Name: product_tags; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.product_tags (
    product_id integer NOT NULL,
    tag_id integer NOT NULL
);


ALTER TABLE public.product_tags OWNER TO admin;

--
-- Name: products; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.products (
    id integer NOT NULL,
    sku text NOT NULL,
    name text NOT NULL,
    description text,
    folder_path text NOT NULL,
    production boolean NOT NULL,
    created_at timestamp without time zone DEFAULT now(),
    category_id integer,
    color text,
    print_time text,
    weight integer,
    rating integer NOT NULL,
    stock_quantity integer,
    reorder_point integer,
    unit_cost integer,
    selling_price integer
);


ALTER TABLE public.products OWNER TO admin;

--
-- Name: products_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.products_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.products_id_seq OWNER TO admin;

--
-- Name: products_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.products_id_seq OWNED BY public.products.id;


--
-- Name: tags; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.tags (
    id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.tags OWNER TO admin;

--
-- Name: tags_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.tags_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.tags_id_seq OWNER TO admin;

--
-- Name: tags_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.tags_id_seq OWNED BY public.tags.id;


--
-- Name: categories id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.categories ALTER COLUMN id SET DEFAULT nextval('public.categories_id_seq'::regclass);


--
-- Name: materials id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.materials ALTER COLUMN id SET DEFAULT nextval('public.materials_id_seq'::regclass);


--
-- Name: products id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.products ALTER COLUMN id SET DEFAULT nextval('public.products_id_seq'::regclass);


--
-- Name: tags id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.tags ALTER COLUMN id SET DEFAULT nextval('public.tags_id_seq'::regclass);


--
-- Data for Name: categories; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.categories (id, name, sku_initials, description) FROM stdin;
\.


--
-- Data for Name: materials; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.materials (id, name) FROM stdin;
\.


--
-- Data for Name: product_materials; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.product_materials (product_id, material_id) FROM stdin;
\.


--
-- Data for Name: product_tags; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.product_tags (product_id, tag_id) FROM stdin;
\.


--
-- Data for Name: products; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.products (id, sku, name, description, folder_path, production, created_at, category_id, color, print_time, weight, rating, stock_quantity, reorder_point, unit_cost, selling_price) FROM stdin;
\.


--
-- Data for Name: tags; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.tags (id, name) FROM stdin;
\.


--
-- Name: categories_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.categories_id_seq', 1, false);


--
-- Name: materials_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.materials_id_seq', 1, false);


--
-- Name: products_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.products_id_seq', 1, false);


--
-- Name: tags_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.tags_id_seq', 1, false);


--
-- Name: categories categories_name_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_name_key UNIQUE (name);


--
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (id);


--
-- Name: categories categories_sku_initials_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_sku_initials_key UNIQUE (sku_initials);


--
-- Name: materials materials_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.materials
    ADD CONSTRAINT materials_pkey PRIMARY KEY (id);


--
-- Name: product_materials product_materials_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.product_materials
    ADD CONSTRAINT product_materials_pkey PRIMARY KEY (product_id, material_id);


--
-- Name: product_tags product_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.product_tags
    ADD CONSTRAINT product_tags_pkey PRIMARY KEY (product_id, tag_id);


--
-- Name: products products_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_pkey PRIMARY KEY (id);


--
-- Name: products products_sku_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_sku_key UNIQUE (sku);


--
-- Name: tags tags_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_pkey PRIMARY KEY (id);


--
-- Name: ix_materials_name; Type: INDEX; Schema: public; Owner: admin
--

CREATE UNIQUE INDEX ix_materials_name ON public.materials USING btree (name);


--
-- Name: ix_product_materials_material_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX ix_product_materials_material_id ON public.product_materials USING btree (material_id);


--
-- Name: ix_product_materials_product_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX ix_product_materials_product_id ON public.product_materials USING btree (product_id);


--
-- Name: ix_product_tags_product_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX ix_product_tags_product_id ON public.product_tags USING btree (product_id);


--
-- Name: ix_product_tags_tag_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX ix_product_tags_tag_id ON public.product_tags USING btree (tag_id);


--
-- Name: ix_products_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX ix_products_id ON public.products USING btree (id);


--
-- Name: ix_tags_name; Type: INDEX; Schema: public; Owner: admin
--

CREATE UNIQUE INDEX ix_tags_name ON public.tags USING btree (name);


--
-- Name: product_materials product_materials_material_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.product_materials
    ADD CONSTRAINT product_materials_material_id_fkey FOREIGN KEY (material_id) REFERENCES public.materials(id);


--
-- Name: product_materials product_materials_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.product_materials
    ADD CONSTRAINT product_materials_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id);


--
-- Name: product_tags product_tags_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.product_tags
    ADD CONSTRAINT product_tags_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id);


--
-- Name: product_tags product_tags_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.product_tags
    ADD CONSTRAINT product_tags_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.tags(id);


--
-- Name: products products_category_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_category_id_fkey FOREIGN KEY (category_id) REFERENCES public.categories(id);


--
-- PostgreSQL database dump complete
--

\unrestrict v8Xosq7eklO5NlHuz8Cre35JGskAzKbNvPtBX0bWgRctz2f4MvnnC7YSSRyspbk

