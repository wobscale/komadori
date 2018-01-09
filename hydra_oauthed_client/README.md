# Hydra Oauthed Client

This crate is a fairly thin wrapper for the swagger-generated rust client (which lives in [../hydra_client](../hydra_client)).

It implements oauth token exchange and refreshing, encapsulates the tokio event loop, and otherwise stays out of the way.

The reason it encapsulates tokio's event loop is because komadori uses rocket, which doesn't have a tokio event loop handy.
It's easier to integrate into komadori by pretending tokio doesn't exist for now. In the future, this abstraction may be pushed up to komadori once rocket does the same.
