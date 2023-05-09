# wfobj-rs
Specification compliant Rust Wavefront Obj file parser/serialization


# Overview
OBJ files were originally created for "The Advanced Visualizer" which was a 3d graphics program developed by Wavefront Technologies in the late 1900's. This technology is now antiquated; however, the format still sees use to this day due to the simplicity of the format and plaintext formatting. There seem to be a few parsers for the Rust programming language for OBJ files; however, they only seem to parse model data, and overlook the rest of the specification. This repo intends to be a fully compliant parser for the obj 3.0 format.

# Requirements
http://www.martinreddy.net/gfx/3d/OBJ.spec      (OBJ File Specification)

The above links appear to be the bulk of the specification; however, a few sections is missing and this does impact the repos ability towards authenticity. 

In order for us to meet the specification requirements we must achieve the following:

    * Vertex Data
        * Geometric Verticies
        * Texture Coordinates
        * Vertex Normals
        * Parameter Space Verticies
    * Freeform geometry
        * Attributes
            * Curve types (Rational and Non-Rational)
                * bmatrix
                * bezier
                * bspline
                * cardinal
                * taylor
            * Variable Basis Matrix Sizes (bmat u & v)
            * Step
            * Degree
        * Elements
            * Curve 
            * 2D Curve
            * Surface
        * Body Statements
            * Parameter (u,v)
            * Trimming Loops
            * Holes
            * Special Curves
            * Special Points
            * End Statement
        * Connectivity
    * Geometry
        * Elements
            * Non-Renderable Geometry
                * Points
                * Lines
            * Renderable Geometry
                * Face
    * Grouping
        * Group names   (Note modern applications can use these interchangably)
        * Object Names  (Note modern applications can use these interchangably)
        * Smoothing Groups
        * Merging Groups
    * Render Attributes (Low priority)
        * LODs
        * Materials & Material Library
        * Shadow Object
        * Raytracing Object
        * Geometry
            * Bevel
            * Color Interpolation
            * Dissolve Interpolations
        * Freeform Geometry
            * Curve Resolution
            * Surface Resolutiom
    * Comments

It is also stated that there is some error checking in the specification. This repo intends to implement as much validation as possible as well.

In addition to this wfobj-rs must be able to construct OBJ files. Since the obj format is a state machine, we must have an intermediary structure that contains the model vertex and index buffers as well as metadata about the individual elements in the obj file.