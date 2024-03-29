﻿// <auto-generated />
using System;
using LeafletAPI.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.EntityFrameworkCore.Storage.ValueConversion;
using Npgsql.EntityFrameworkCore.PostgreSQL.Metadata;

#nullable disable

namespace LeafletAPI.Migrations
{
    [DbContext(typeof(BlobDbContext))]
    partial class BlobDbContextModelSnapshot : ModelSnapshot
    {
        protected override void BuildModel(ModelBuilder modelBuilder)
        {
#pragma warning disable 612, 618
            modelBuilder
                .HasAnnotation("ProductVersion", "6.0.3")
                .HasAnnotation("Relational:MaxIdentifierLength", 63);

            NpgsqlModelBuilderExtensions.UseIdentityByDefaultColumns(modelBuilder);

            modelBuilder.Entity("LeafletAPI.Models.BlobEntity", b =>
                {
                    b.Property<Guid>("Id")
                        .HasColumnType("uuid");

                    b.Property<Guid>("AccountId")
                        .HasColumnType("uuid");

                    b.Property<string>("BlobType")
                        .IsRequired()
                        .HasColumnType("text");

                    b.Property<byte[]>("Content")
                        .IsRequired()
                        .HasColumnType("bytea");

                    b.Property<DateTime>("LastChanged")
                        .HasColumnType("timestamp with time zone");

                    b.HasKey("Id", "AccountId");

                    b.ToTable("Blobs");
                });
#pragma warning restore 612, 618
        }
    }
}
