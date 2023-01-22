Try some markdown formula.

$$
\begin{aligned}
\delta^L &= \nabla_{a^L}C \odot \sigma'(z^L) & (1) \\[0.7em]
\delta^l &= (w^{l+1})^T \delta^{l+1} \odot \sigma'(z^l) & (2) \\[0.7em]
\frac{\partial C }{\partial b_j^l}  &= \delta^l_j & (3) \\[0.7em]
\frac{\partial C }{\partial w_{kj}^l} &= \delta_k^l a_j^{l-1} & (4)
\end{aligned}
$$

